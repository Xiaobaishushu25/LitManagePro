import {defineStore} from "pinia";
import {ref} from "vue";
import {DocumentTags} from "../components/main/main-type.ts";

const useDocStore = defineStore('docs', ()=>{
    const docs = ref<DocumentTags[]>([]);
    function addNewDoc(doc: DocumentTags) {
        docs.value.push(doc);
    }
    return {
        docs,
        addNewDoc,
    }
})
export default useDocStore