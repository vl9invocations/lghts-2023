<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const lightStatus = ref("");
const lightState = ref();

async function get_light_status() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  lightStatus.value = await invoke("get_light_statuses");
}

async function turn_switch() {
  await invoke("turn_switch");
}

</script>

<template>
  <div class="light-unit_status" :class="{ '--on': lightState }"></div>
  <div class="light-unit_info">
    <p>N: Garage light</p>
    <p>{{ lightStatus }}</p>
    <!-- <p>{{ lightState }}</p> -->
  </div>
  <div class="light-unit_control-btns">

    <form class="light-unit_switch" @click="turn_switch">
      SWITCH
    </form>

    <form class=" light-unit_switch" @click="get_light_status">
      CHECK
    </form>
  </div>

  <div class="light-unit_bulbs">
    <div class="light-unit_connection"></div>
    <!-- <div class="light-unit_switch">
        SET
      </div> -->
    <div class="light-unit_connection --status"></div>
  </div>
</template>
