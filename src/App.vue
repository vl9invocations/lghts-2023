<script setup>
import GetLightStatus from "./components/GetLightStatus.vue";


</script>

<template>
  <div data-tauri-drag-region class="ligth-unit_header">
    <div class="titlebar-button">
      <img src="/src-tauri/icons/Square30x30Logo.png" alt="" srcset="">
      <p>LC-R55</p>
    </div>
    <div id="titlebar-close" class="titlebar-button">
      <svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24">
        <path fill="currentColor"
          d="M19 3H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V5a2 2 0 0 0-2-2m0 16H5V5h14zM17 8.4L13.4 12l3.6 3.6l-1.4 1.4l-3.6-3.6L8.4 17L7 15.6l3.6-3.6L7 8.4L8.4 7l3.6 3.6L15.6 7z" />
      </svg>
      <!-- <img src="https://api.iconify.design/icon-park/close-one.svg" /> -->
    </div>
  </div>


  <div class=" light-unit">
    <GetLightStatus />
  </div>
</template>
