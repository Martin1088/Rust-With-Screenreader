<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri"

    let result ="";
    async function mrbs_test() {
        result = await invoke("mrbs_today");
        console.log(result);
        JSON.stringify(result)
    }
</script>
<div>
    <p>{ result}</p>
</div>