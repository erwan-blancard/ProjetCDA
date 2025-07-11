/** use IndexedDB API to store account names based on account id
 * to reduce amount of requests to make to the api
 * @type IDBDatabase */
let DB;

const DBNAME = "RandomiCache";
const DBVERSION = 2;
const ACCOUNT_NAMES_STORE = "accounts";
/** in ms */
const EXPIRATION_TIME = 60 * 60 * 1000	// 1 hour


// const request = indexedDB.open(DBNAME, DBVERSION);

// request.onerror = (event) => {
// 	console.log(`Error opening cache DB: ${event.target.error?.message}`);
// };

// request.onupgradeneeded = (event) => {
//     DB = event.target.result;

//     DB.createObjectStore(ACCOUNT_NAMES_STORE, { keyPath: "id" /* AccountDTO wrapped in { "account", "date" } obj */ });

// 	DB.onerror = (event) => {
// 		console.error(`Database error: ${event.target.error?.message}`);
// 	};
// };

let dbReady = new Promise((resolve, reject) => {
	const request = indexedDB.open(DBNAME, DBVERSION);

	request.onerror = (event) => {
		console.error(`Error opening cache DB: ${event.target.error?.message}`);
		reject(event.target.error);
	};

	request.onupgradeneeded = (event) => {
		const db = event.target.result;
		if (!db.objectStoreNames.contains(ACCOUNT_NAMES_STORE)) {
			db.createObjectStore(ACCOUNT_NAMES_STORE, { keyPath: "account.id" });
		}
	};

	request.onsuccess = (event) => {
		DB = event.target.result;
		resolve(DB);
	};
});


export async function getAccountFromStore(account_id) {
	await dbReady;

	return new Promise((resolve, reject) => {
		const store = DB
			.transaction(ACCOUNT_NAMES_STORE, "readwrite")
			.objectStore(ACCOUNT_NAMES_STORE);
		
		const request = store.get(account_id);
		request.onerror = (event) => {
			console.error(`Error when retrieving account in cache db: ${event.target.error?.message}`);
			// reject(event.target.error);
			resolve(null);
		};
		request.onsuccess = (event) => {
			const data = event.target.result;

			if (data == null) {
				resolve(null);
			} else {
				// if expired, remove record
				if (data.date + EXPIRATION_TIME < Date.now()) {
					const requestDelete = store.delete(account_id);
					requestDelete.onerror = (event) => {
						console.error(`Error when removing account in cache db (expired): ${event.target.error?.message}`);
					}
					// reject("Record Expired");
					resolve(null);
				} else {
					resolve(data.account);
				}
			}
		};

	});
}


export async function storeAccount(accountDTO) {
	await dbReady;

	return new Promise((resolve, reject) => {
		const store = DB
			.transaction(ACCOUNT_NAMES_STORE, "readwrite")
			.objectStore(ACCOUNT_NAMES_STORE);
		
		const record = { account: accountDTO, date: Date.now() };

		const request = store.put(record);
		request.onerror = (event) => {
			console.error(`Error when storing account in cache db: ${event.target.error?.message}`);
			reject(event.target.error);
		};

		request.onsuccess = () => {
			resolve();
		}
	});
}
