<template>
  <div class="center-wrapper">
    <p v-if="!checkedIn">Please scan your ticket to receive your boarding pass.</p>
    <div>
      <qrcode-capture @detect="onDetect" :capture="selected.value" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { QrcodeCapture } from "vue-qrcode-reader";
const checkedIn = ref(false);
const result = ref<string>('');
const options = [
  { text: 'force file dialog', value: false }
];
const selected = ref(options[0]);

interface detectedCode {
  rawValue: string;
}

const onDetect = (detectedCodes: detectedCode[]) => {
  result.value = JSON.stringify(
      detectedCodes.map((code: any) => code.rawValue)
  );
  checkedIn.value = true;
}
</script>


<style scoped>
.center-wrapper {
  display: flex;
  flex-direction: column;
  align-items: center;
  height: 100vh;
}
</style>
