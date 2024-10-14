<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/tauri';

  let dynamicContent: string = 'Initial content';

  // Function to update the dynamic content by invoking the Tauri command
  async function updateContent() {
    try {
      // Call the Tauri command to get a random character
      const randomChar: string = await invoke<string>('dynamic');
      dynamicContent += randomChar;

      // Programmatically focus the element to make it accessible for screen readers and braille displays
      const element = document.getElementById('dynamic-content');
      if (element) {
        element.setAttribute('aria-busy', 'true'); // Indicate the element is updating
        element.textContent = dynamicContent;
        element.setAttribute('aria-busy', 'false'); // Reset busy state to indicate update completion
        element.setAttribute('aria-relevant', 'all'); // Set relevant attributes to ensure braille updates
        element.focus();
        element.blur(); // Force an update by removing focus after focusing
      }
    } catch (error) {
      console.error('Error updating content:', error);
    }
  }

  // Set up the interval to update content every 10 seconds
  onMount(() => {
    const interval = setInterval(updateContent, 10000);
    return () => clearInterval(interval); // Cleanup when component is destroyed
  });
</script>

<main>
  <p
    id="dynamic-content"
    role="status"
    aria-live="assertive"
    aria-atomic="true"
    aria-relevant="all"
    tabindex="-1">
    {dynamicContent}
  </p>
</main>

<style>
  /* Optional styling for the dynamic content */
  #dynamic-content {
    outline: none; /* Hide the outline when focused programmatically */
  }
</style>