const path				= require('path');
const log				= require('@whi/stdlog')(path.basename( __filename ), {
    level: process.env.LOG_LEVEL || 'fatal',
});


const fs				= require('fs');
const expect				= require('chai').expect;
const { expect_reject }                 = require('./utils.js');
const { HoloHash }			= require('@whi/holo-hash');
const { Holochain }			= require('@whi/holochain-backdrop');
const { ConductorError,
	EntryNotFoundError,
	DeserializationError,
	CustomError,
	...hc_client }			= require('@whi/holochain-client');

const json				= require('@whi/json');

const { backdrop }			= require('./setup.js');


const delay				= (n) => new Promise(f => setTimeout(f, n));
const INTEGRITY_TEMPLATE_PATH		= path.join(__dirname, "../../dna/workdir/dna/integrity-template.dna");
let clients;


function basic_tests () {
    it("should create an entry with valid text", async function () {
	this.timeout( 10_000 );

        let addr			= new HoloHash( await clients.alice.call( "integrity-template-dna", "integrity-template-coordinator-zome", "create", "Hello, world!" ) );
	log.normal("New entry address: %s", String(addr) );

    });
}

function errors_tests () {
    it("should fail creating an entry with known-invalid text", async function () {
        this.timeout( 10_000 )

        await expect_reject( async () => {
            await clients.alice.call( "integrity-template-dna", "integrity-template-coordinator-zome", "create", "invalid text 1" );
        }, ConductorError, "Source chain error: InvalidCommit error: invalid thing1" );
    });
}

describe("Zome: Integrity Template", () => {

    const holochain			= new Holochain();

    before(async function () {
	this.timeout( 30_000 );

	clients				= await backdrop( holochain, {
	    "integrity-template-dna": INTEGRITY_TEMPLATE_PATH,
	}, [
	    "alice",
	], {
	    "parse_entities": false,
	});
    });

    describe("Basic", basic_tests.bind( this, holochain ) );
    describe("Errors", errors_tests.bind( this, holochain ) );

    after(async () => {
	await holochain.stop();
	await holochain.destroy();
    });

});
