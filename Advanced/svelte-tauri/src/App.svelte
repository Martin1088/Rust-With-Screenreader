<script lang="ts">
  import Greet from './lib/Greet.svelte'
  import today from './today.json'
</script>

<main class="container">
  <h1>Raumplaner Demo</h1>
  <div> 
    <container class="wider mobile">
      <h1>Raumplaner</h1>
        {#each today as today }
        <h2>{ today.area_name }</h2>
        <select name="allrooms" id="allroooms">
          {#each today.rooms as room}
            <option value="{ room.room_name }">{ room.room_name }</option>
          {/each}
        </select>
        <input type="button" value="join">
        <button type="button">unjoin for {today.area_name}</button>
        <ul>
          {#each today.rooms as room}
            <li>{ room.room_name }</li>
            <li>{ room.participants}</li>
          {/each}
        </ul>
         {/each}
    </container>
  </div>
  
 
  <div class="row">
    <Greet />
  </div>


</main>

<style>
    div {
      margin: 25px;
      display: flex;
      flex-direction: column;
      align-items: center;
    }
  
    @media only screen and (max-width: 620px) {
      container.mobile {
        width: 300px;
        word-wrap: break-word;
      }
    }
 
</style>