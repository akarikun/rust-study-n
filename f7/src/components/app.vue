<template>
  <f7-app v-bind="f7params">
    <f7-view main class="safe-areas" url="/"></f7-view>
  </f7-app>
</template>
<script setup>
import { ref, onMounted } from 'vue';
import { f7, f7ready } from 'framework7-vue';
import { io } from "socket.io-client";

import routes from '../js/routes.js';
import store from '../js/store';
import * as $ from '../js/utils.js'

const f7params = ref({
  name: 'study-n', // App name
  theme: 'auto', // Automatic theme detection
  // App store
  store: store,
  // App routes
  routes: routes,
  view: {
    browserHistory: false,
    browserHistorySeparator: ''
  }
});


onMounted(() => {
  f7ready(() => {
    // Call F7 APIs here
    console.log(f7.colors);

    const ioc = io("/ws");
    ioc.on('connect', () => {
      window.addEventListener($.study_server_msg, (data) => {
        // console.log(data.detail)
        ioc.emit($.study_server_msg, data.detail)
      }, false);
    });
    ioc.on($.study_msg_resp, data => {
      console.log($.study_msg_resp, data);
      $.dispatchPageMessage(data)
    })
  });
});

const props = defineProps({
  f7route: Object,
  f7router: Object,
})
</script>