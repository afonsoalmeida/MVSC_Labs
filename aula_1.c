#include <stdio.h>
#include <stdlib.h>
#include <pthread.h>

/*
    Neste trabalho optou-se por fazer uma pesquisa serial em vez de pesquisa binária
    porque como estamos a lidar com listas ligadas, não faz muito sentido "partir"
    a lista em 2 em cada iteração da pesquisa binária, como foi discutido pelos
    colegas no Slack.
    Na main, começa-se por criar a cabeça da lista a NULL (essencialmente não existe lista ainda)
    e pela criação de 2 threads. Um deles irá inserir nºs pares de 2 a 10 (t_insert_even) e o outro irá
    inserir nºs ímpares de 1 a 10 (t_insert_odd). Ambos os métodos chamam o método insert, que recebe a
    cabeça da lista e o nó a inserir, e encontra o sítio onde o colocar.
    A lista está sempre ordenada de forma crescente.
    Apesar de o resultado correto ser

    1  2  3  4  5  6  7  8  9  10

    visto não haver nenhum tipo de coordenação entre threads, é possível obter resultados "inesperados"
    como

    1  2  3  4  5  6  7  9 ;

    1  2  3  5  7  8  9  10 ;

    ...(outros semelhantes)

    A razão de algumas vezes se obterem estes resultados inesperados é que visto que não há nenhum controlo
    de acesso e modificação a um meio partilhado (cada nó da lista), uma thread poderá num instante começar
    a fazer alterações num nó desatualizado e escrever por cima das alterações da outra thread, eliminando resultados.
    No primeiro exemplo, ambas as threads tentam inserir um nº a seguir ao 7 ao mesmo tempo. No entanto, a thread dos
    nºs pares é mais rápida e insere 8 e 10 antes que a outra thread insira o 9. O que acontece é que no contexto da
    thread dos nºs ímpares, o 7 é o nº mais baixo abaixo do 9 e portanto o este deverá ser a seguir ao 7, e reescreve
    a ligação entre 7 e 8 (criada pelo outro thread) e cria uma entre 7 e 9. Como a ligação entre 7 e 8 é perdida,
    também é perdida a ligação entre 8 e 10.
*/

struct Node {
    int data;
    struct Node* next;
};

void printList(struct Node* head)
{
    struct Node* temp = head;
    while (temp != NULL) {
        printf("%d  ", temp->data);
        temp = temp->next;
    }
    printf("\n");
}

void insert(struct Node** head, struct Node* toInsert) {
    struct Node* curr;

    if(*head == NULL || (*head)->data >= toInsert->data) {
        toInsert->next = *head;
        *head = toInsert;
    } else {
        curr = *head;
        while(curr->next != NULL && curr->next->data < toInsert->data) {
            curr = curr->next;
        }
        toInsert->next = curr->next;
        curr->next = toInsert;
    }
}

void* t_insert_even(void* head) {
    for(int i=2; i<=10; i+=2) {
        struct Node* toInsert = (struct Node*)malloc(sizeof(struct Node));
        toInsert->data = i;
        toInsert->next = NULL;
        insert((struct Node**)head, (struct Node*)toInsert);
    }
}

void* t_insert_odd(void* head) {
    for(int i=1; i<=10; i+=2) {
        struct Node* toInsert = (struct Node*)malloc(sizeof(struct Node));
        toInsert->data = i;
        toInsert->next = NULL;
        insert((struct Node**)head, (struct Node*)toInsert);
    }
}

int main() {
    struct Node* head = NULL;
    //head = (struct Node*)malloc(sizeof(struct Node));
    //head->data = -1;
    //head->next = NULL;
    //Fazia parte do enunciado
    /*
    struct Node* tail = NULL;
    tail = (struct Node*)malloc(sizeof(struct Node));
    head->data=4;
    head->next=tail;
    tail->data=7;
    tail->next=NULL;
    */

    pthread_t t1,t2;
    pthread_create(&t1,NULL,t_insert_even, &head);
    pthread_create(&t2,NULL,t_insert_odd, &head);

    pthread_join(t1, NULL);
    pthread_join(t2, NULL);

    printList(head);

    free(head);
}
