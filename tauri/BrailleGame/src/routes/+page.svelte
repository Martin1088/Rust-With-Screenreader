<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';

  let dynamicContent: string = 'Initial content';

  async function updateContent() {
    try {
      // Call the Tauri command to get a random character
      const randomChar: string = await invoke<string>('dynamic');
      dynamicContent += randomChar;

      const element = document.getElementById('dynamic-content');
      if (element) {
        // Update the content and apply focus to trigger accessibility updates
        element.textContent = dynamicContent;
        element.setAttribute('aria-busy', 'true'); // Indicate update in progress
        element.focus(); // Focus to force update
        element.setAttribute('aria-busy', 'false'); // Mark update as complete

        // Blurring and refocusing can sometimes help the braille display detect the change
        setTimeout(() => {
          element.blur(); // Remove focus briefly
          element.focus(); // Refocus
        }, 100);
      }
    } catch (error) {
      console.error('Error updating content:', error);
    }
  }

  // Set up the interval to update content every 10 seconds
  onMount(() => {
    const interval = setInterval(updateContent, 10000);
    return () => clearInterval(interval); // Cleanup interval on component unmount
  });
  
// function updateClock() {
//   const now = new Date();
//   document.getElementById("clock-hours").textContent = now.getHours();
//   document.getElementById("clock-mins").textContent =
//     `0${now.getMinutes()}`.substr(-2);
// }

// /* first run */
// updateClock();

// /* update every minute */
// setInterval(updateClock, 60000);
 </script>

 <main>
   <p
     id="dynamic-content"
     role="log"
     aria-live="assertive"
     aria-atomic="true"
     tabindex="-1">
     {dynamicContent}
   </p> <br>
//   <div id="clock" role="timer" aria-live="polite">
//     <span id="clock-hours"></span>
//     <span id="clock-mins"></span>
//   </div>

// </main>

<style>
  #dynamic-content {
    outline: none; /* Hide outline for better visual presentation */
  }
</style>