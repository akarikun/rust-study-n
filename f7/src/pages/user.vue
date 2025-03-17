<template>
    <f7-page class="page-user">
        <f7-list dividers-ios strong-ios outline-ios>
            <f7-list-item class="user-item" link="#">
                <img src="/icons/avatar.webp" width="80" height="80" alt="avatar" />
                <ul>
                    <li>大帅哥</li>
                    <li>等级: N3</li>
                </ul>
            </f7-list-item>
            <f7-list-item title="录入" link="/n3-post" />
        </f7-list>
        <f7-block>
            <div class="descript">日语学习@v1.0.0</div>
        </f7-block>
    </f7-page>
</template>
<script setup>

</script>
<style scoped>
img {
    border-radius: 12px;
    margin-right: 10px;
}

.descript {
    width: 100%;
    text-align: center;
}

</style>