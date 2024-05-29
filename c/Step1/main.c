//
//  DoubleLinkedList.c
//  DoubleLinkedList
//
//  Created by 김진호 on 2023/03/01.
//
// TODO: something

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
int InsertAtTail(void *pParam, const char *(*pfGetKey)(void *));
int InsertAtHead(void *pParam, const char *(*pfGetKey)(void *));

// For structure user
typedef struct USERDATA {
  char szName[64]; // Key
  char szPhone[64];
  const char *(*pfGetKey)(void *);
} USERDATA;

// User makes a function that gets the key
const char *GetKeyUserData(USERDATA *pUser) { return pUser->szName; }
const char *GetPhoneUserData(USERDATA *pUser) { return pUser->szPhone; }

int CreateUserData(const char *pszName, const char *pszPhone) {
  USERDATA *pNewData = (USERDATA *)malloc(sizeof(USERDATA));
  memset(pNewData, 0, sizeof(USERDATA));
  strlcpy(pNewData->szName, pszName, sizeof(pNewData->szName));
  strlcpy(pNewData->szPhone, pszPhone, sizeof(pNewData->szName));
  return InsertAtTail(pNewData, (void *)GetKeyUserData);
}

// For structure developer - make a rule of struct
typedef struct NODE {
  void *pData; // Abstract Data (this can be any struct)

  // function pointer that both returns char pointer and get no data
  const char *(*pfGetKey)(void *);
  // const char *(*pfGetOther)(void *);

  struct NODE *prev; // Struct
  struct NODE *next;
} NODE;

NODE *g_pHead; // pointer for head
NODE *g_pTail; // pointer for tail
int g_nSize;   // indicate size

void InitList(void) {
  g_pHead = (NODE *)malloc(sizeof(NODE));
  g_pTail = (NODE *)malloc(sizeof(NODE));
  // malloc - allocate memory on the heap as the size of the object
  // and return the address of the first byte
  g_nSize = 0;

  memset(g_pHead, 0, sizeof(NODE));
  memset(g_pTail, 0, sizeof(NODE));
  // fill with 0 (initialize)

  g_pHead->next = g_pTail;
  g_pTail->prev = g_pHead;
  // make linked list by indicating head and tail
}

void ReleaseList(void) {
  NODE *pTmp = g_pHead;   // copy address of current head
  while (pTmp != NULL) {  // repeat until current head is not NULL
    NODE *pDelete = pTmp; // save heads' address before delete
    pTmp = pTmp->next;    // make head point to next node (connect)
    printf("free(%p)\n", pDelete);
    free(pDelete->pData); // free data in previous node
    free(pDelete);        // free previous head
  }

  g_pHead = NULL;
  g_pTail = NULL;
  g_nSize = 0;
  // reset all of default config

  puts("ReleaseList()");
}

void PrintList(void) {
  printf("PrintList(): g_nSize: %d g_pHead [%p], g_pTail [%p]\n", g_nSize,
         g_pHead, g_pTail);
  printf("[%p], %p, [%p]\n", g_pHead->prev, g_pHead, g_pHead->next);
  NODE *pTmp = g_pHead->next; // point to current NODE
  int i = 0;
  while (pTmp != g_pTail) {
    // puts(pTmp->pszData);
    printf("index: %d  [%p] '%s %s', %p, [%p]\n", i++, pTmp->prev,
           pTmp->pfGetKey(pTmp->pData), pTmp->pfGetKey(pTmp->pData), pTmp,
           pTmp->next);
    pTmp = pTmp->next;
  }
  printf("[%p], %p, [%p]\n\n", g_pTail->prev, g_pTail, g_pTail->next);
}

//===============================================
// pParam : User can allocate memory and set values
//===============================================
NODE *MakeNode(void *pParam, const char *(fnGetKey)(void *)) { // abstract
  NODE *pNewNode = malloc(sizeof(NODE));
  memset(pNewNode, 0, sizeof(NODE));
  // Initiallize Data
  pNewNode->pData = pParam;
  pNewNode->pfGetKey = fnGetKey;
  // pNewNode->pData =
  //     (USERDATA *)malloc(sizeof(USERDATA)); // this is depend on USERDATA
  //     type.
  return pNewNode;
}

int InsertAfter(NODE *pDstNode, void *pParam, const char *(*pfGetKey)(void *)) {
  NODE *pNewNode = MakeNode(pParam, pfGetKey);
  pNewNode->prev = pDstNode;
  pNewNode->next = pDstNode->next;
  pDstNode->next->prev = pNewNode;
  pDstNode->next = pNewNode;
  g_nSize++;
  return g_nSize;
}

int InsertBefore(NODE *pNode, void *pParam, const char *(*pfGetKey)(void *)) {
  NODE *pNewNode = MakeNode(pParam, pfGetKey);

  // Initiallize Structure
  pNewNode->prev = pNode->prev;
  pNewNode->next = pNode;

  pNode->prev->next = pNewNode;
  pNode->prev = pNewNode;

  g_nSize++;
  return g_nSize;
}

// int를 반환하면, index를 반환 할 수 있어서 좋다.
int InsertAtHead(void *pParam, const char *(*pfGetKey)(void *)) {
  return InsertAfter(g_pHead, pParam, pfGetKey);
}

int InsertAtTail(void *pParam, const char *(*pfGetKey)(void *)) {
  return InsertBefore(g_pTail, pParam, pfGetKey);
}

NODE *FindNode(const char *pszKey) {
  NODE *pTmp = g_pHead->next; // start from first
  while (pTmp != NULL) {
    if (strcmp(pTmp->pfGetKey(pTmp->pData), pszKey) == 0) {
      return pTmp;
    }
    pTmp = pTmp->next;
  }
  return 0;
}

int DeleteNode(const char *pszKey) {
  NODE *pNode = FindNode(pszKey);

  pNode->prev->next = pNode->next;
  pNode->next->prev = pNode->prev;

  free(pNode->pData);
  free(pNode);
  g_nSize--;
  printf("DeleteNode(): %p\n", pNode);
  return 0;
}

int GetSize(void) { return g_nSize; }

int GetLength(void) { return GetSize(); }

int IsEmpty(void) { return g_nSize == 0; }

NODE *GetAt(int idx) {
  // Get from start
  NODE *pTmp = g_pHead->next; // index 0
  while (pTmp != g_pTail) {
    if (idx-- == 0) {
      return pTmp;
    }
    pTmp = pTmp->next;
  }
  return NULL;
}

int InsertAt(int idx, void *pParam, const char *(*pfGetKey)(void *)) {
  // Insert at idx
  NODE *pTmp = GetAt(idx);
  if (pTmp == NULL) {
    return -1;
  }

  return InsertBefore(pTmp, pParam, pfGetKey);
}

int main(void) {
  /* const int a = 0; */
  /* const int a = a + 1; */

  InitList();

  CreateUserData("Name01", "010-1234-5678");
  CreateUserData("Name02", "010-1111-2222");

  printf("%d hello", 1);
  // DeleteNode("Test03");
  // InsertAtHead("Test03");
  // int result = InsertAt(2, "Test AT 00");
  // printf("result: %d\n", result);

  // if (FindNode("Test01") != NULL) {
  //   puts("FindNode(\"Test01\")");
  // }

  PrintList();

  // InsertAtTail("Test04");

  // InsertAtTail("Test06");

  // PrintList();

  ReleaseList();

  return 0;
}
