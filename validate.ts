import { Scenario } from "@holochain/tryorama";
import { ActionHash } from "@holochain/client";
import test from "tape";
import { FIXTURE_DNA_URL } from "./dna/workdir/dna/index.js";

interface HolochainError {
    type: string;
    data: { type: string, data: string };
}

test("Validate store entry - creating an entry with valid text should succeed", async (t) => {
    const scenario = new Scenario();
    const player = await scenario.addPlayerWithHapp([{ path: FIXTURE_DNA_URL.pathname }]);
    const actionHash: ActionHash = await player.cells[0].callZome({ zome_name: "coordinator", fn_name: "create", payload: "valid text" });
    console.log( 'integrity create: ' + JSON.stringify( actionHash, null, 4 ))


    t.equal(actionHash.length, 39, "create entry returns a valid action hash");
    await scenario.cleanUp();
});

test("Validate store entry - creating an entry with invalid text should fail", async (t) => {
    const scenario = new Scenario();
    try {
        const player = await scenario.addPlayerWithHapp([{ path: FIXTURE_DNA_URL.pathname }]);
        const l = await player.cells[0].callZome({ zome_name: "coordinator", fn_name: "create", payload: "invalid text" });
        console.log('e', l);
        t.fail();
    } catch (e) {
        const hcError = e as HolochainError;
        t.ok(hcError.data.data.includes("invalid thing"));
    }
    await scenario.cleanUp();
});

// test("Validate register update", async (t) => {
//     const scenario = new Scenario();
//     const player = await scenario.addPlayerWithHapp([{ path: FIXTURE_DNA_URL.pathname }]);
//     const actionHash: ActionHash = await player.cells[0].callZome({ zome_name: "coordinator", fn_name: "create", payload: "valid text" });
//     await player.cells[0].callZome({ zome_name: "coordinator", fn_name: "create", payload: {} });
// });
