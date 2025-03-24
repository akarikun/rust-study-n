<template>
    <f7-page :page-content="false">
        <f7-navbar title="N3红蓝宝书1000题" back-link="返回"></f7-navbar>
        <f7-tabs class="tab" animated swipeable @tab:show="tab_show">
            <f7-tab :id="`tab-n3-1000-${item.id}`" :data-id="item.id" v-for="(item, index) in form" :key="item.id"
                class="page-content">
                <f7-card outline-md>
                    <f7-card-header valign="bottom">{{ item.index }}.
                        <span v-if="item.index > 1"><f7-button
                                sheet-open=".demo-sheet-swipe-to-close">查看上一题</f7-button></span>
                    </f7-card-header>
                    <f7-card-content class="card-main">
                        <f7-block strong v-html="item.content"></f7-block>
                        <br /><br /><br /><br /><br />
                        <f7-list strong-ios outline-ios dividers-ios>
                            <f7-list-item v-for="({ text, result }, index2) in format_result(item)" :key="index2">
                                <f7-button large fill v-if="result" :tab-link="`#tab-n3-1000-${item.id + 1}`">{{ text
                                }}</f7-button>
                                <f7-button large fill v-else @click="handle_select">{{ text }}</f7-button>
                            </f7-list-item>
                        </f7-list>
                    </f7-card-content>
                    <f7-block v-if="show_remark">
                        <br />
                        <f7-block>正确答案：{{format_result(item).filter(x => x.result)[0].text}}</f7-block>
                        <hr />
                        <f7-block v-html="item.remark"></f7-block>
                    </f7-block>
                </f7-card>
            </f7-tab>
        </f7-tabs>
        <f7-sheet class="demo-sheet-swipe-to-close" style="height: auto" swipe-to-close push backdrop>
            <f7-page-content>
                <f7-card outline-md>
                    <f7-card-content>
                        <f7-block strong v-html="`${modal_form.index}. ${modal_form.content}`"
                            class="line-bottom card-main"></f7-block>
                        <br /><br />
                        <f7-block strong v-html="modal_form.remark" class="line-bottom card-main"></f7-block>
                        <br /><br />
                        <f7-list strong-ios outline-ios dividers-ios>
                            <f7-list-item v-for="({ text, result }, index2) in format_result(modal_form)" :key="index2">
                                <f7-button large fill v-if="result">{{ text }}</f7-button>
                                <f7-button large v-else>{{ text }}</f7-button>
                            </f7-list-item>
                        </f7-list>
                    </f7-card-content>
                </f7-card>
            </f7-page-content>
        </f7-sheet>
    </f7-page>
</template>

<script setup>
import { ref, onMounted, watch, triggerRef } from 'vue';
import { f7, f7ready } from 'framework7-vue';
import store from '../js/store';
import * as $ from '../js/utils';

const props = defineProps({
    f7route: Object,
    f7router: Object,
})

const form_init = () => {
    return [{
        id: 0,
        level: 3,
        index: 0,
        content: "",
        a: "", b: "", c: "", d: "",
        result: 1,
        type: 0,
        remark: ""
    }]
}

const form = ref(form_init().filter(x => x.id > 0));
const modal_form = ref(form_init()[0]);
const show_remark = ref(false);

const tab_show = (e) => {
    // console.log(e);
    show_remark.value = false;
    store.dispatch('update_index', 3);
    let id = e.target.getAttribute('data-id');
    id = parseInt(id) - 1;
    modal_form.value = form.value.filter(x => x.id == id)[0];
    send_message();
}

const send_message = () => {
    let level = 3;
    let { index } = store.state.booklist['n3'];
    $.MSG.send_message('get_study_list', { level, index, step: 2 })
}

const handle_select = () => {
    show_remark.value = true;
}

const format_result = (item) => {
    return [
        { text: 'A: ' + item.a, result: item.result == 1 },
        { text: 'B: ' + item.b, result: item.result == 2 },
        { text: 'C: ' + item.c, result: item.result == 3 },
        { text: 'D: ' + item.d, result: item.result == 4 },
    ]
}

onMounted(() => {
    $.MSG.register_page(({ msg, data, status }) => {
        if (msg == 'get_study_list_resp') {
            if (status == 1) {
                let { index } = store.state.booklist['n3'];
                let arr = data.filter(x => x.id >= index);
                if (arr.length > 0) {
                    if (form.value.length > 0) {
                        let id = form.value[form.value.length - 1].id
                        arr = arr.filter(x => x.id > id);
                    }
                    form.value.push(...arr);
                }
                if (index > 1) {
                    let val = data.filter(x => x.id == index - 1)[0];
                    modal_form.value = val;
                }
            }
        }
    });
    send_message();
})
</script>
<style lang="less" scoped>
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

.line-bottom {
    border: 1px solid grey;
}
</style>