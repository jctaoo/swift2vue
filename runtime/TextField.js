import SwiftCommon from "./SwiftCommon.js";

export default {
  components: { SwiftCommon },
  name: "TextField",
  setup() {
    return {}
  },
  template: `<SwiftCommon>
    <template #child="{ setRef }">
      <input type="text" class="text-field built-in" :ref="(el) => setRef(el)" />
    </template>
  </SwiftCommon>`
};
