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
                        <f7-block-title>请选择</f7-block-title>
                        <br />
                        <f7-list strong-ios outline-ios dividers-ios>
                            <f7-list-item radio v-for="(item2, index2) in sel_arr" v-model="sel_val"
                                :name="`checkbox-${item}`">
                                <label>{{ format_index(index2) }}: <span>{{ item2 }}</span></label>
                                <label>错误</label>
                            </f7-list-item>
                        </f7-list>
                    </f7-card-content>
                    <!-- <f7-card-footer> -->
                    <!-- <span><a href="#">查看上一题</a></span> -->
                    <!-- <span>本题正确答案: <b style="color:tomato">A</b></span> -->
                    <!-- </f7-card-footer> -->
                </f7-card>
                <f7-block>
                    本题提示
                </f7-block>
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
const selected = ref('home');
const sel_arr = ref([]);
const sel_val = ref(null)

watch(sel_val, (newVal, oldVal) => {
    console.log(newVal)
    console.log(oldVal)
}, { deep: true })

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
onMounted(() => {
    sel_arr.value = ['あこがれる', 'みだれる', 'めぐまれる', 'たおれる'];

    dispatchEvent(new CustomEvent('study_msg', {
        detail: {
            msg: 'select',
            data: { level: 3, index: 1 },
        },
    }));

    // const ioc = io("/ws");
    // ioc.on('connect', () => {
    //     ioc.emit('select', { level: 3, index: 1 })
    // });
    // ioc.on('select_resp', data => {
    //     console.log(data);
    // })
})
</script>
<style lang="less" scoped>
* {
    // background:red;
}

u {
    color: red;
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