<template>
    <f7-page :page-content="false">
        <f7-list>
            <f7-list-item :title="format_content(item.index, item.content)" v-for="(item, index) in list" :key="index"
                :link="`/n3-post?id=${item.id}&level=${item.level}&index=${item.index}`"></f7-list-item>
        </f7-list>
    </f7-page>
</template>
<script setup>
import { ref, onMounted } from 'vue';
import { f7, f7ready } from 'framework7-vue';
import * as $ from '../js/utils';

const list = ref([])

const format_content = (index, content) => {
    return '(' + index + ') ' + content.replace(/<[^>]+>(.*)<\/[^>]+>/gim, '$1').replace(/&nbsp;/gim, ' ');
}

onMounted(() => {
    $.MSG.register_page(({ status, msg, data }) => {
        if (msg == 'get_study_list_resp') {
            list.value = data;
        }
    });
    setTimeout(() => {
        $.MSG.send_message('get_study_list', {
            level: 3
        });
    }, 2000)
})

</script>
<style scoped>
.list {
    margin-top: 0;
}
</style>