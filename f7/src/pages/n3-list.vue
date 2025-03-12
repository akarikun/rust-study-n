<template>
    <f7-page>
        <f7-list contacts-list strong-ios dividers-ios>
            <f7-list-group>
                <f7-list-item title="1-50" group-title></f7-list-item>
                <f7-list-item title="index2" v-for="(item2, index2) in arr" :key="index2"></f7-list-item>
            </f7-list-group>
        </f7-list>
    </f7-page>
</template>
<script setup>
import { ref, onMounted } from 'vue';
import { f7, f7ready } from 'framework7-vue';

const arr_init = ()=>new Array(2).fill([]);
const arr = ref(arr_init())

</script>
<style scoped></style>