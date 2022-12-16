<template>
  <q-page class="row justify-evenly q-pa-xs">
    <q-card class="fit">
      <q-card-section>
        <q-scroll-area
          style="height: 600px; max-width: 100%"
          ref="scrollAreaRef"
        >
          <q-chat-message
            class="q-ma-md"
            :name="x.name"
            :text="[x.text]"
            :sent="x.sent"
            :avatar="
              !x.sent ? 'https://cdn.quasar.dev/img/avatar2.jpg' : undefined
            "
            v-for="(x, i) in messages"
            v-bind:key="i" /></q-scroll-area
      ></q-card-section>
    </q-card>
  </q-page>
</template>

<script lang="ts">
import { defineComponent, ref } from 'vue';
export default defineComponent({
  name: 'IndexPage',
  components: {},
  setup() {
    const scrollAreaRef = ref(null);
    const messages = ref([] as { name: string; text: string; sent: boolean }[]);
    setInterval(() => {
      fetch('api/hi').then((resp) => {
        if (resp.ok) {
          resp.json().then((data) => {
            const ask = { name: 'me', text: 'timestamp?', sent: true };
            const answer = {
              name: 'backend-app',
              text: data.timestamp,
              sent: false,
            };
            messages.value.push(ask);
            messages.value.push(answer);
          });
        } else {
          messages.value.push({
            name: 'me',
            text: `status:${resp.status},local timestamp:${Date.now()}`,
            sent: true,
          });
        }
        const scroll = scrollAreaRef.value as any;
        scroll && scroll.setScrollPercentage('vertical', 1, 100);
      });
    }, 1000);
    return { scrollAreaRef, messages };
  },
});
</script>
