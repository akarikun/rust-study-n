<template>
    <f7-page>
        <f7-navbar back-link="返回" title="录入"></f7-navbar>
        <f7-list strong-ios dividers-ios inset>
            <f7-block-title>添加</f7-block-title>
            <f7-list-input label="所属级别" :value="form.level" @change="form.level = $event.target.value" type="select">
                <option value="4">N4-N5</option>
                <option value="3">N3(目前只支持N3)</option>
                <option value="2">N2</option>
                <option value="1">N1</option>
            </f7-list-input>

            <f7-list-input label="题序" type="text" :value="form.index" @input="form.index = $event.target.value"
                placeholder="题序" clear-button></f7-list-input>

            <f7-list-item>
                <f7-text-editor placeholder="输入题目" :value="form.content" @input="form.content = $event.target.innerHTML"
                    :custom-buttons="customButtons" :buttons="[['('], [')'], ['___']]"></f7-text-editor>
            </f7-list-item>

            <f7-list-input label="答案A" type="text" :value="form.a" @input="form.a = $event.target.value"
                placeholder="答案A" clear-button></f7-list-input>
            <f7-list-input label="答案B" type="text" :value="form.b" @input="form.b = $event.target.value"
                placeholder="答案B" clear-button></f7-list-input>
            <f7-list-input label="答案C" type="text" :value="form.c" @input="form.c = $event.target.value"
                placeholder="答案C" clear-button></f7-list-input>
            <f7-list-input label="答案D" type="text" :value="form.d" @input="form.d = $event.target.value"
                placeholder="答案D" clear-button></f7-list-input>

            <f7-list-input label="正确答案" :value="form.result" @change="form.result = $event.target.value" type="select">
                <option value="1">A</option>
                <option value="2">B</option>
                <option value="3">C</option>
                <option value="4">D</option>
            </f7-list-input>

            <f7-list-input label="类型备注" :value="form.type" @change="form.type = $event.target.value" type="select">
                <option value="0">一般</option>
                <option value="1">语法难</option>
            </f7-list-input>

            <f7-list-item>
                <f7-text-editor placeholder="备注" :value="form.remark" @input="form.remark = $event.target.innerHTML" />
            </f7-list-item>

            <f7-list-item>
                <f7-button large fill @click="post_data">提交</f7-button>
            </f7-list-item>
        </f7-list>
    </f7-page>
</template>

<script setup>
import { ref, toRefs, toRaw, onMounted, watch } from 'vue';
import { f7, f7ready } from 'framework7-vue';
import * as $ from '../js/utils';

const form_init = () => {
    return {
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

onMounted(() => {
    f7ready(() => {
        setTimeout(() => {
            $.MSG.get_last_index(form.value.level);
        }, 1000)
        $.MSG.register_page_msg(({ data, status }) => {
            form.value.index = data + 1;
        });
    })
});
watch(form, (newVal, oldVal) => {
    if (newVal.value && newVal.value.level != oldVal.value.level) {
        $.MSG.get_last_index(newVal.level)
    }
}, { deep: true })

const customButtons = ref({
    '(': {
        content: '<b>(_</b>',
        onClick() {
            document.execCommand('insertText', false, ' ( ');
            document.execCommand('underline', false, null);
        },
    },
    ')': {
        content: '<b>_)</b>',
        onClick() {
            document.execCommand('underline');
            document.execCommand('insertText', false, '  ) ');
        },
    },
    '___': {
        content: '<b>___</b>',
        onClick() {
            document.execCommand('insertText', false, '_______');
        },
    },
});

const post_data = () => {
    console.log("提交数据：", JSON.stringify(form.value, null, 2));    
    $.MSG.post_study(toRaw(form.value));
    form.value = form_init()
    f7.notification.create({
        icon: '<i class="icon icon-f7"></i>',
        title: '日语学习',
        //titleRightText: 'now',
        // subtitle: '操作成功',
        text: '操作成功',
        closeTimeout: 3000,
    }).open();
};
</script>

<style lang="less" scoped>
.text-editor {
    width: 100%;
    padding: 0;
    margin: 0;
}

.item-inner {
    >a {
        width: 100%;
    }
}
</style>
