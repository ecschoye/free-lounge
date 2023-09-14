<template>
  <div class="">
    <NavBar />
    <slot />
  </div>
</template>

<script setup lang="ts">
import NavBar from "~/components/NavBar.vue";
</script>