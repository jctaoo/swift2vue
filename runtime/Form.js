import SwiftCommon from "./SwiftCommon.js";

export default {
  components: { SwiftCommon },
  name: "Form",
  setup() {
    return {}
  },
  template: `<SwiftCommon>
    <template #child="{ setRef }">
      <form class="form built-in" :ref="(el) => setRef(el)">
        <slot></slot>
      </form>
    </template>
  </SwiftCommon>`
};
