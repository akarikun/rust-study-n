<template>
    <f7-page :page-content="false">
        <f7-navbar title="N3红蓝宝书1000题" back-link="返回"></f7-navbar>
        <f7-tabs class="tab" animated swipeable @tab:show="tab_show">
            <f7-tab :id="`tab-n3-1000-${item}`" v-for="item in form" :key="item.id" class="page-content">
                <f7-card outline-md>
                    <f7-card-header valign="bottom">{{ item.index }}. <span><f7-button
                                :tab-link="`#tab-n3-1000-${item.id - 1}`">查看上一题</f7-button></span></f7-card-header>
                    <f7-card-content class="card-main">
                        <f7-block strong v-html="item.content"></f7-block>
                        <br /><br /><br /><br /><br />
                        <f7-list strong-ios outline-ios dividers-ios>
                            <f7-list-item><f7-button large fill @click="handle_select('a', item)">A:{{ item.a
                            }}</f7-button></f7-list-item>
                            <f7-list-item><f7-button large fill @click="handle_select('b', item)">B:{{ item.b
                            }}</f7-button></f7-list-item>
                            <f7-list-item><f7-button large fill @click="handle_select('c', item)">C:{{ item.c
                            }}</f7-button></f7-list-item>
                            <f7-list-item><f7-button large fill @click="handle_select('d', item)">D:{{ item.d
                            }}</f7-button></f7-list-item>
                        </f7-list>
                    </f7-card-content>
                </f7-card>
            </f7-tab>
        </f7-tabs>
    </f7-page>
</template>

<script setup>
import { ref, onMounted, watch } from 'vue';
import { f7, f7ready } from 'framework7-vue';
import * as $ from '../js/utils';

// const props = defineProps({
//     f7route: Object,
//     f7router: Object,
// })

const form_init = () => {
    return {
        id: 0,
        level: 3,
        index: 0,
        content: "",
        a: "", b: "", c: "", d: "",
        result: 1,
        type: 0,
        remark: ""
    }
}

const form = ref(form_init());

const sel_arr = ref([]);

const format_index = (item) => {
    switch (item) {
        case 0: return 'A';
        case 1: return 'B';
        case 2: return 'C';
        case 3: return 'D';
        default: return;
    }
}

const tab_show = (tab) => {
    console.log(tab);
}

const handle_select = (result, item) => {

}

onMounted(() => {
    sel_arr.value = ['あこがれる', 'みだれる', 'めぐまれる', 'たおれる'];

    $.MSG.register_page(({ msg, data, status }) => {
        if (msg == 'get_study_list_resp') {
            if (status) {
                form.value = data;
            }
        }
    });
    $.MSG.send_message('get_study_list', { level: 3, index: 1, step: 2 })
})
</script>
<style lang="less" scoped>
// * {
//     background:red;
// }

.list {
    a {
        width: 100%;
    }
}

.card {

    .card-header,
    .card-footer {
        color: white;
        background: #2196f3;
    }

    .card-header {
        a {
            color: white;
        }
    }
}
</style>