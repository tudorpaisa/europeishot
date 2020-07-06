<template>
  <el-row :gutter="20" class="table-row">
    <el-col :span="18">
      {{ city }}
    </el-col>
    <el-col :span="6">
      {{ temp }}
    </el-col>
  </el-row>
</template>

<script>
export default {
  name: "TableRow",
  props: {
    city: String,
    temp: Number
  }
};
</script>

<style lang="scss">
.table-row {
  border: solid;
}
</style>
