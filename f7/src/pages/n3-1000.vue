<template>
    <f7-page :page-content="false">
        <f7-navbar title="N3红蓝宝书1000题" back-link="返回"></f7-navbar>
        <f7-tabs class="tab" animated swipeable @tab:show="tab_show">
            <f7-tab :id="`tab-n3-1000-${item.id}`" :data-id="item.id" v-for="(item, index) in form" :key="item.id"
                class="page-content">
                <f7-card outline-md>
                    <f7-card-header valign="bottom">{{ item.index }}.
                        <span v-if="item.index > 1"><f7-button sheet-open=".demo-sheet-swipe-to-close"
                                @click="prev_handle">查看上一题</f7-button></span>
                    </f7-card-header>
                    <f7-card-content class="card-main">
                        <f7-block strong v-html="item.content"></f7-block>
                        <br /><br /><br /><br /><br />
                        <f7-list strong-ios outline-ios dividers-ios>
                            <f7-list-item v-for="({ text, result }, index2) in format_result(item)" :key="index2">
                                <f7-button large fill v-if="result" :tab-link="`#tab-n3-1000-${item.id + 1}`">{{ text
                                }}</f7-button>
                                <f7-button large fill v-else>{{ text }}</f7-button>
                            </f7-list-item>
                        </f7-list>
                    </f7-card-content>
                </f7-card>
            </f7-tab>
        </f7-tabs>
        <f7-sheet class="demo-sheet-swipe-to-close" style="height: auto" swipe-to-close push backdrop>
            <f7-page-content>
                <f7-card outline-md>
                    <f7-card-content>
                        <f7-block strong v-html="modal_form.content" class="line-bottom card-main"></f7-block>
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
import { ref, onMounted, watch } from 'vue';
import { f7, f7ready } from 'framework7-vue';
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



const form = ref(form_init());
const modal_form = ref(form_init()[0]);

const tab_show = (e) => {
    console.log(e);
    let id = e.target.getAttribute('data-id');
    id = parseInt(id) - 1;
    modal_form.value = form.value.filter(x => x.id == id)[0];
}

const format_result = (item) => {
    return [
        { text: 'A: ' + item.a, result: item.result == 1 },
        { text: 'B: ' + item.b, result: item.result == 2 },
        { text: 'C: ' + item.c, result: item.result == 3 },
        { text: 'D: ' + item.d, result: item.result == 4 },
    ]
}

const prev_handle = () => {

}

onMounted(() => {
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
.line-bottom{
    border:1px solid grey;
}
</style>