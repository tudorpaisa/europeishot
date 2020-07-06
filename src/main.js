import Vue from "vue";
import VueRouter from "vue-router";
//import "./styles/index.css";
import ElementUI from "element-ui";
import "element-ui/lib/theme-chalk/index.css";
import locale from "element-ui/lib/locale/lang/en";
import App from "./App.vue";
import TempTable from "./components/TempTable.vue";
import Info from "./components/Info.vue";

const routes = [
  { path: "/", component: TempTable },
  { path: "/info", component: Info }
];

const router = new VueRouter({
  routes
});

Vue.config.productionTip = false;
Vue.use(ElementUI, { locale });
Vue.use(VueRouter);

new Vue({
  router: router,
  render: h => h(App)
}).$mount("#app");
