// Browser-only contract checks; no backend/model capability is inferred from these.
const {chromium}=require(process.env.PLAYWRIGHT_MODULE || 'playwright');
const {pathToFileURL}=require('node:url');
const path=require('node:path');
const assert=require('node:assert/strict');
let browser;
(async()=>{
 browser=await chromium.launch({headless:true,args:['--no-sandbox']});
 const page=await browser.newPage({viewport:{width:1280,height:900}});
 const errors=[];page.on('pageerror',e=>errors.push(e.message));
 await page.route('https://**/*',r=>r.abort());
 await page.goto(pathToFileURL(path.join(__dirname,'index.html')).href);
 const click=async a=>page.locator(`[data-action="${a}"]`).first().click();
 const data=()=>page.evaluate(()=>JSON.parse(localStorage.getItem('aws-context-spindle-v8')));
 const current=async()=>{const s=await data();return s.projects.find(p=>p.id===s.project)};
 // Keyboard dialog handling and project selection.
 await click('tab:ui');await click('create');await page.keyboard.press('Escape');assert.equal(await page.locator('[role="dialog"]').count(),0);
 await click('ui-project:agent-support');await click('nav:board');
 await click('settings');await click('privacy');await page.locator('#privacy-sample').fill('Contains API keys');await click('privacy-test');assert.match(await page.locator('#privacy-result').textContent(),/Blocked/);await click('close');
 // Malformed and reserved board metadata is rejected; literal privacy covers metadata too.
 await click('new-note');await page.locator('#note-title').fill('A safe title');await page.locator('#note-text').fill('A useful note.');await page.locator('.modal summary').click();await page.locator('#note-attrs').fill('{"id":"overridden"}');await click('note-save');assert.match(await page.locator('#note-error').textContent(),/reserved/);
 await page.locator('#note-attrs').fill('{"private":"API keys"}');await click('note-save');assert.match(await page.locator('#note-error').textContent(),/exclusion/);await click('close');
 // Rollback retires unsupported notes and preserves independent sources.
 await click('nav:sessions');await click('session:S-104');await click('rollback:S-104');await page.locator('#rollback-mode').selectOption('bounded');await click('rollback-run:S-104');await page.waitForTimeout(1400);let p=await current();assert(!p.notes.some(n=>!n.retracted&&n.sources.includes('S-104')));assert(p.notes.some(n=>!n.retracted&&n.sources.includes('S-106')));await click('close');
 // Subscription lifecycle ends on leave and cannot resume without an active session.
 await click('tab:cli');await page.locator('#command').fill('/context board subscribe Watch /delivery for status changes.');await page.locator('#command').press('Enter');assert((await current()).subscribed);await page.locator('#command').fill('/context leave');await page.locator('#command').press('Enter');await click('tab:ui');assert.equal((await current()).subscribed,false);await click('tab:cli');await page.locator('#command').fill('/context board subscribe Watch /delivery.');await page.locator('#command').press('Enter');assert.equal((await current()).subscribed,false);await click('tab:ui');
 // Archive is reversible and doesn't remove knowledge. Reload retains local edits.
 await click('settings');await page.locator('.modal summary').click();await click('archive');await click('archive-confirm');assert((await current()).archived);const count=(await current()).notes.length;await page.reload();await click('tab:ui');await click('ui-project:agent-support');assert((await current()).archived);assert.equal((await current()).notes.length,count);await click('settings');await page.locator('.modal summary').click();await click('restore');assert.equal((await current()).archived,false);
 assert.deepEqual(errors,[]);console.log('PASS: keyboard dialog, project selection, privacy, reserved metadata, rollback, subscription lifecycle, archive/restore, reload persistence; no JS errors.');
 await browser.close();
})().catch(async e=>{console.error(e);if(browser)await browser.close();process.exitCode=1});
