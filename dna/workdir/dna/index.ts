import { dirname } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const dnaPath = dirname(fileURLToPath(import.meta.url)) + "/dna.dna";
export const FIXTURE_DNA_URL = pathToFileURL(dnaPath);
