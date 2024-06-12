import SwiftCommon from "./SwiftCommon.js";

export default {
  components: { SwiftCommon },
  name: "List",
  setup() {
    return {}
  },
  template: `<SwiftCommon>
    <template #child="{ setRef }">
      <ul class="list built-in" :ref="(el) => setRef(el)"><slot></slot></ul>
    </template>
  </SwiftCommon>`
};
