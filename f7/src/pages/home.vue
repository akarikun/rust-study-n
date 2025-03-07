<template>
  <f7-page :page-content="false">
    <f7-navbar title="日语学习"></f7-navbar>
    <f7-toolbar bottom tabbar icons :scrollable="false">
      <f7-link tab-link="#tab-home-1" tab-link-active text="首页" icon-ios="f7:house" icon-md="material:house" />
      <f7-link tab-link="#tab-home-2" text="录入" icon-ios="f7:cloud_upload" icon-md="material:cloud_upload" />
      <f7-link tab-link="#tab-home-3" text="我的" icon-ios="f7:face_smiling" icon-md="material:face_smiling" />
    </f7-toolbar>

    <f7-tabs swipeable>
      <f7-tab id="tab-home-1" class="page-content" tab-active>
        <f7-block>
          <f7-block>
            <f7-button large fill color="orange"> N4-N5(数据暂未录入) </f7-button>
          </f7-block>
          <f7-block>
            <f7-button large fill color="blue" @click="props.f7router.navigate('/n3-1000')"> N3红蓝宝书1000题 </f7-button>
          </f7-block>
          <f7-block>
            <f7-button large fill color="green"> N2(数据暂未录入) </f7-button>
          </f7-block>
          <f7-block>
            <f7-button large fill color="red"> N1(数据暂未录入) </f7-button>
          </f7-block>
        </f7-block>
      </f7-tab>
      <f7-tab id="tab-home-2" class="page-content">
        <f7-block>
          <p>Tab 2 content</p>

        </f7-block>
      </f7-tab>
      <f7-tab id="tab-home-3" class="page-content">
        <f7-block>
          <p>Tab 3 content</p>

        </f7-block>
      </f7-tab>
    </f7-tabs>
  </f7-page>
</template>
<script setup>
import Login from '../pages/login.vue'
const props = defineProps({
  f7route: Object,
  f7router: Object,
})
</script>
<style scoped>
/* #app .page {
  height:calc(100% - 48px);
} */
</style>