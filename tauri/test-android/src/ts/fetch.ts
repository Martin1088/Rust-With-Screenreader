import { invoke } from "@tauri-apps/api/core";
import { projektdir } from "./store";


export async function testdefault() {
    await invoke("testdir")
        .then((res) => projektdir.set(res))
        .catch((e) => projektdir.set(e) );
  }