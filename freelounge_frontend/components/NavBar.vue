<template>
  <nav class="navbar">
    <ul class="navbar-list">
      <li class="navbar-item">
        <NuxtLink to="/">Home</NuxtLink>
      </li>
      <li class="navbar-item">
        <NuxtLink to="/checkin">Check in</NuxtLink>
      </li>
    </ul>
  </nav>
</template>

<script setup lang="ts">
// TypeScript code can go here if needed
</script>

<style scoped>
.navbar {
  background-color: white;
  color: black;
  padding: 1rem;
}

.navbar-list {
  list-style: none;
  padding: 0;
  margin: 0;
  display: flex;
  justify-content: center;
  align-items: center;
}

.navbar-item {
  margin: 0 1rem;
}
</style>
