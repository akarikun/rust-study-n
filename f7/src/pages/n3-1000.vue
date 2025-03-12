<template>
    <f7-page :page-content="false">
        <f7-navbar title="N3红蓝宝书1000题" back-link="返回"></f7-navbar>
        <f7-tabs class="tab" animated swipeable @tab:show="tab_show">
            <f7-tab :id="`tab-n3-1000-${item}`" v-for="item in 10" :key="item" class="page-content">
                <f7-card outline-md>
                    <f7-card-header valign="bottom">{{ item }}. <span><f7-button
                                :tab-link="`#tab-n3-1000-${item - 1}`">查看上一题</f7-button></span></f7-card-header>
                    <f7-card-content>
                        <f7-block strong>
                            <p>
                                日本では野球選手に<u>憧れる</u>子どもたちが多い。
                            </p>
                        </f7-block>
                        <br /><br /><br /><br /><br />
                        <f7-list strong-ios outline-ios dividers-ios>
                            <f7-list-item v-for="(item2, index2) in sel_arr" :key="index2">
                                <f7-button large fill @click="handle_select(item, index2 + 1)">{{ format_index(index2) }}:
                                    {{ item2 }}</f7-button>
                            </f7-list-item>
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

// const props = defineProps({
//     f7route: Object,
//     f7router: Object,
// })
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

const handle_select = (level, result) => {
    debugger;
}

onMounted(() => {
    sel_arr.value = ['あこがれる', 'みだれる', 'めぐまれる', 'たおれる'];

    // dispatchEvent(new CustomEvent('study_msg', {
    //     detail: {
    //         msg: 'get_last_index',
    //         data: { level: 3, index: 1 },
    //     },
    // }));
    // dispatchServerMessage('get_last_index',{})
})
</script>
<style lang="less" scoped>
* {
    // background:red;
}

u {
    color: red;
}

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