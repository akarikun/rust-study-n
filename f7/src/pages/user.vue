<template>
    <f7-page class="page-user">
        <f7-list dividers-ios strong-ios outline-ios>
            <f7-list-item class="user-item" link="#" v-if="!login_status" @click="login_handle">
                <svg xmlns="http://www.w3.org/2000/svg" width="80" height="80"
                    viewBox="0 0 32 32"><!-- Icon from Carbon by IBM - undefined -->
                    <path fill="none"
                        d="M8.007 24.93A4.996 4.996 0 0 1 13 20h6a4.996 4.996 0 0 1 4.993 4.93a11.94 11.94 0 0 1-15.986 0M20.5 12.5A4.5 4.5 0 1 1 16 8a4.5 4.5 0 0 1 4.5 4.5">
                    </path>
                    <path fill="currentColor"
                        d="M26.749 24.93A13.99 13.99 0 1 0 2 16a13.9 13.9 0 0 0 3.251 8.93l-.02.017c.07.084.15.156.222.239c.09.103.187.2.28.3q.418.457.87.87q.14.124.28.242q.48.415.99.782c.044.03.084.069.128.1v-.012a13.9 13.9 0 0 0 16 0v.012c.044-.031.083-.07.128-.1q.51-.368.99-.782q.14-.119.28-.242q.451-.413.87-.87c.093-.1.189-.197.28-.3c.071-.083.152-.155.222-.24ZM16 8a4.5 4.5 0 1 1-4.5 4.5A4.5 4.5 0 0 1 16 8M8.007 24.93A4.996 4.996 0 0 1 13 20h6a4.996 4.996 0 0 1 4.993 4.93a11.94 11.94 0 0 1-15.986 0">
                    </path>
                </svg>
                <ul style="color:grey;">
                    <li>未登录</li>
                    <li>等级: - </li>
                </ul>
            </f7-list-item>
            <f7-list-item class="user-item" link="#" v-else>
                <img src="/icons/avatar.webp" width="80" height="80" alt="avatar" />
                <ul>
                    <li>大帅哥</li>
                    <li>等级: N3 </li>
                </ul>
            </f7-list-item>
            <f7-list-item title="查看录入列表(N3)" link="/n3-list?level=3" />
            <f7-list-item title="录入(N3)" class="upload_study" link="/n3-post?level=3" v-if="login_status" />
        </f7-list>
        <f7-block>
            <div class="descript">日语学习@v1.0.0</div>
        </f7-block>
    </f7-page>
</template>
<script setup>
import { ref, onMounted, watch } from 'vue';
import { f7, f7ready } from 'framework7-vue';
import store from '../js/store';
const login_status = ref(null);// store.state.user

onMounted(() => {

})
const login_handle = () => {
    f7.dialog.alert('功能开发中...');
    // location.href= '/wechat/mp/login.do?state=wechat_mp'
    // login_status.value = {};
    // store.dispatch('set_user_info', true);
}

</script>
<style scoped>
img {
    border-radius: 12px;
    margin-right: 16px;
}

svg {
    border-radius: 12px;
    margin-right: 16px;
}

.descript {
    width: 100%;
    text-align: center;
}

.upload_study {
    color: red;
}
</style>