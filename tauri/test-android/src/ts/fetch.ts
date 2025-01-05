import { invoke } from "@tauri-apps/api/core";
import { projektdir, projektenv } from "./store";


export async function testdefault() {
    await invoke("testdir")
        .then((res) => projektdir.set(res))
        .catch((e) => projektdir.set(e));
}

export async function testenv() {
    await invoke("testenv")
        .then((res) => projektenv.set(res))
        .catch((e) => projektenv.set(e));
}
