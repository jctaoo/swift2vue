import SwiftCommon from "./SwiftCommon.js";

export default {
  components: { SwiftCommon },
  name: "ForEach",
  setup() {
    return {}
  },
  template: `<SwiftCommon>
    <template #child="{ setRef }">
      <li class="for-each built-in" :ref="(el) => setRef(el)"><slot></slot></li>
    </template>
  </SwiftCommon>`
};
