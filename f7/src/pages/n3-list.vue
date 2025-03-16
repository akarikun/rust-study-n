<template>
    <f7-page :page-content="false">
        <f7-list contacts-list strong-ios dividers-ios>
            <f7-list-group v-for="(item, index) in list" :key="index">
                <f7-list-item :title="`${(index * page + 1)} - ${((index + 1) * page)}`" group-title></f7-list-item>
                <f7-list-item :title="format_content(item2.content)" v-for="(item2, index2) in item"
                    :key="index2"></f7-list-item>
            </f7-list-group>
        </f7-list>
    </f7-page>
</template>
<script setup>
import { ref, onMounted } from 'vue';
import { f7, f7ready } from 'framework7-vue';
import * as $ from '../js/utils';

const page = ref(2)
const list = ref([[]])

const split_array = (arr, size) => {
    const result = [];
    for (let i = 0; i < arr.length; i += size) {
        result.push(arr.slice(i, i + size));
    }
    return result;
}
const format_content = (content) => {
    return content.replace(/<[^>]+>(.*)<\/[^>]+>/, RegExp.$1 || content);
}
onMounted(() => {
    $.MSG.register_page(({ status, msg, data }) => {
        if (msg == 'get_study_list_resp') {
            list.value = split_array(data, page.value);
            console.log(list.value);
        }
    });
    setTimeout(() => {
        $.MSG.send_message('get_study_list', {
            level: 3
        });
    }, 1000)
})

</script>
<style scoped>
#root .page-content {
    display: none;
    padding: 0;
}
</style>