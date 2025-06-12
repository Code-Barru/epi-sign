import postgres from 'postgres';
import { ulid } from "ulid";
import puppeteer from 'puppeteer';

async function get_cookies () {
    const browser = await puppeteer.launch({
        headless: false,
        args: [
            '--no-sandbox',
            '--disable-setuid-sandbox',
        ],
    });

    const page = await browser.newPage();

    await page.goto('https://intra.epitech.eu', { waitUntil: 'networkidle2' });

    await page.waitForFunction(() => {
        return document.cookie.split('; ').length > 7;
    }, { timeout: 20000 });

    let cookies = await browser.defaultBrowserContext().cookies();
    if (cookies.length != 8) {
        console.warn(`Only found ${cookies.length} cookies after waiting.`);
    }
    cookies = cookies.map(cookie => {
        if (cookie.expires && typeof cookie.expires === 'number') {
            cookie.expires = Math.trunc(cookie.expires);
        }
        return cookie;
    });

    await browser.close();
    return cookies;
};

async function save_cookies(cookies) {
    const formattedDate = new Date().toISOString().split('T')[0];
    const id = ulid();

    // Replace with your actual Postgres URL
    const POSTGRES_URL = process.env.DATABASE_URL || 'postgres://postgres:postgres@localhost/postgres';
    const sql = postgres(POSTGRES_URL);

    try {
        await sql`
            INSERT INTO cookies (id, date, cookie_data)
            VALUES (${id}, ${formattedDate}, ${JSON.stringify(cookies)})
        `;
        console.log('Cookies uploaded to Postgres.');
    } catch (err) {
        console.error('Failed to upload cookies:', err);
    } finally {
        await sql.end();
    }
}

async function main() {
    console.log('Starting cookie retrieval...');
    const cookies = await get_cookies();
    console.log(`Retrieved ${cookies.length} cookies.`);
    await save_cookies(cookies);
    console.log('Cookie upload complete.');
    process.exit(0);
}

main()