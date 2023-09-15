<template>
  <nav class="navbar">
    <ul class="navbar-list">
      <div class="link-section">
        <li class="navbar-item">
          <NuxtLink to="/">Home</NuxtLink>
        </li>
        <li class="navbar-item">
          <NuxtLink to="/checkin">Check in</NuxtLink>
        </li>
        <li class="navbar-item">
          <NuxtLink to="/schedule">Flight schedule</NuxtLink>
        </li>
      </div>
      <div class="time-section">
        <li class="navbar-item navbar-time">
          12:30 01/10/2023
        </li>
      </div>
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
  justify-content: space-between; /* This will push elements to the corners */
  align-items: center;
}

.link-section, .time-section {
  display: flex;
  align-items: center;
}

.navbar-item {
  margin: 0 1rem;
}

.navbar-time {
  font-weight: bold;
  color: gray;
}
</style>
