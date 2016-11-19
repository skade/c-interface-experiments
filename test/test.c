#include "../include/core.h"

int main (int argc, char const *argv[])
{
        CoreClient* c = create_ios_client();
        inspect(c);
        destroy(c);
        CoreClient* a = create_android_client();
        inspect(a);
        destroy(a);
}
