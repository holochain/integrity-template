const path				= require('path');
const log				= require('@whi/stdlog')(path.basename( __filename ), {
    level: process.env.LOG_LEVEL || 'fatal',
});


const fs				= require('fs');
const expect				= require('chai').expect;
const { HoloHash }			= require('@whi/holo-hash');
const { Holochain }			= require('@whi/holochain-backdrop');
const json				= require('@whi/json');

const { backdrop }			= require('./setup.js');


const delay				= (n) => new Promise(f => setTimeout(f, n));
const INTEGRITY_TEMPLATE_PATH		= path.join(__dirname, "../../dna/workdir/dna/integrity-template.dna");
let clients;


function basic_tests () {
    it("should create an entry", async function () {
	this.timeout( 10_000 );

        let addr			= new HoloHash( await clients.alice.call( "integrity-template-dna", "integrity-template-coordinator-zome", "create", "Hello, world!" ) );
	log.normal("New entry address: %s", String(addr) );

    });
  /*
    const input			= Buffer.from("Somewhere over the rainbow");
    let memory_addr;
    let memory;

    it("should create a memory using 'save_bytes'", async function () {
	this.timeout( 10_000 );

	let addr			= new HoloHash( await clients.alice.call( "memory", "mere_memory", "save_bytes", input ) );
	log.normal("New memory address: %s", String(addr) );

	memory_addr			= addr;
    });

    it("should get a memory using 'retrieve_bytes'", async function () {
	this.timeout( 10_000 );

	memory				= await clients.alice.call( "memory", "mere_memory", "get_memory", memory_addr );
	log.normal("New memory: %s", json.debug(memory) );
    });

    it("should calculate hash of the memory bytes", async function () {
	this.timeout( 10_000 );

	{
	    let hash			= await clients.alice.call( "memory", "mere_memory", "calculate_hash", input );
	    log.normal("Calculated hash: %s", hash );

	    expect( hash		).to.deep.equal( memory.hash );
	}

	{
	    let hash			= await clients.alice.call( "memory", "mere_memory", "calculate_hash", Buffer.from("hello world") );
	    log.normal("Calculated hash: %s", hash );

	    expect( hash		).to.equal("b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9");
	}
    });

    it("should find a memory based on the hash", async function () {
	this.timeout( 10_000 );

	{
	    let exists			= await clients.alice.call( "memory", "mere_memory", "memory_exists", input );
	    log.normal("Memory exists: %s", exists );

	    expect( exists		).to.be.true;
	}

	{
	    let exists			= await clients.alice.call( "memory", "mere_memory", "memory_exists", Buffer.from("hello world") );
	    log.normal("Memory exists: %s", exists );

	    expect( exists		).to.be.false;
	}
    });
  */
}

function errors_tests () {
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
