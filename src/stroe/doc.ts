import {defineStore} from "pinia";
import {ref} from "vue";
import {DocumentTags} from "../components/main/main-type.ts";

const useDocStore = defineStore('docs', ()=>{
    const docs = ref<DocumentTags[]|null>(null);
    const currentSelectDoc = ref<DocumentTags>();
    function addNewDoc(doc: DocumentTags) {
        docs.value!.push(doc);
    }
    function setAllDocs(_docs: DocumentTags[]) {
        docs.value = _docs;
    }
    function setCurrentSelectDoc(doc: DocumentTags) {
        currentSelectDoc.value = doc;
    }
    function updateDoc(doc: DocumentTags){
        docs.value!.forEach((item,index)=>{
            if(item.id === doc.id){
                docs.value![index] = doc;
            }
        })
        if (currentSelectDoc.value?.id === doc.id){
            currentSelectDoc.value = doc;
        }
    }
    return {
        docs,
        currentSelectDoc,
        setAllDocs,
        setCurrentSelectDoc,
        addNewDoc,
        updateDoc
    }
})
export default useDocStore