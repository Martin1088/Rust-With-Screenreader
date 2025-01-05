
import { invoke } from "@tauri-apps/api/core";
import { projekttest, projektenv } from "./store";


export async function testdefault() {
  let t = "braille test hallo";
  await invoke("send_to_voiceover_and_braille", { text: t })
    .then((res) => projekttest.set(res))
    .catch((e) => projekttest.set(e));
}

