<template>
    <f7-page ptr :ptr-mousewheel="true" @ptr:refresh="loadMore">
        <f7-navbar back-link="返回" :title="`${$.format_level(f7route.query.level)}列表`"></f7-navbar>
        <f7-list strong-ios dividers-ios outline-ios>
            <f7-list-item :title="format_content(item.index, item.content)" v-for="(item, index) in list" :key="index"
                :link="`/n3-post?id=${item.id}&level=${item.level}&index=${item.index}`" />
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

const props = defineProps({
    f7route: Object,
    f7router: Object
});

const loading = ref(true);

onMounted(() => {
    $.MSG.register_page(({ status, msg, data }) => {
        if (msg == 'get_study_list_resp') {
            list.value = data;
            loading.value = false;
        }
    });
    setTimeout(() => {
        $.MSG.send_message('get_study_list', {
            level: 3
        });
    }, 1000)
})

const loadMore = (done) => {
    loading.value = true;
    $.MSG.send_message('get_study_list', {
        level: 3
    });

    setTimeout(() => { if (!loading.value) { done(); } }, 1000)
}
</script>
<style scoped></style>