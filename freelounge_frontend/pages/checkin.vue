<template>
  <div class="center-wrapper">
    <p v-if="!checkedIn">
      Please scan your ticket to receive your boarding pass.
    </p>
    <p v-if="isSubmitted">{{ result }}</p>
    <div>
      <qrcode-capture @detect="onDetect" :capture="selected.value" />
    </div>
  </div>
</template>

<script setup lang="ts">
import axios from 'axios';
import { ref } from "vue";
import { QrcodeCapture } from "vue-qrcode-reader";

const checkedIn = ref(false);
const isSubmitted = ref(false);
const payload = ref<string>('');
const result = ref<string>('');
const selected = ref({ text: 'force file dialog', value: false });

interface detectedCode {
  format: string;
  rawValue: string;
}

const onDetect = async (qrString: detectedCode[]) => {
  isSubmitted.value = false;
  if (qrString.length === 0) return;

  const { rawValue } = qrString[0];
  payload.value = rawValue;
  checkedIn.value = true;

  try {
    const response = await axios.post('http://127.0.0.1:7878/QR', rawValue);
    isSubmitted.value = true;
    if (response.status === 200) {
      result.value = "Check-in successful: " + response.data;
    } else {
      result.value = "Check-in failed: " + response.data;
    }
  } catch (error) {
    isSubmitted.value = true;
    result.value = "There was an error with the check-in.";
    console.error("There was an error with the check-in:", error);
  }
};
</script>

<style scoped>
.center-wrapper {
  display: flex;
  flex-direction: column;
  align-items: center;
  height: 100vh;
}
</style>
