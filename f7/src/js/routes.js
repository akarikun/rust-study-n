
import HomePage from '../pages/home.vue';

import LoginPage from '../pages/login.vue';
import N31000Page from '../pages/N3-1000.vue';

import DynamicRoutePage from '../pages/dynamic-route.vue';
import NotFoundPage from '../pages/404.vue';

var routes = [
  {
    path: '/',
    component: HomePage,
  },
  {
    path:'/login',
    component: LoginPage,
  },
  {
    path:'/n3-1000',
    component: N31000Page,
  },
  {
    path: '/dynamic-route/blog/:blogId/post/:postId/',
    component: DynamicRoutePage,
  },
  {
    path: '(.*)',
    component: NotFoundPage,
  },
];

export default routes;
