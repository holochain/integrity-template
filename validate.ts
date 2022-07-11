import { Scenario } from "@holochain/tryorama";
import test from "tape";
import { FIXTURE_DNA_URL } from "./dna/workdir/dna/index.js";

test("Validate", async (t) => {
    const scenario = new Scenario();
    try {
        const player = await scenario.addPlayerWithHapp([{ path: FIXTURE_DNA_URL.pathname }]);
        const response = await player.cells[0].callZome({ zome_name: "coordinator", fn_name: "create" });
        console.log('response', response);
    } catch (e) {
        console.error("eerer", e);
    }
    await scenario.shutDown();
})