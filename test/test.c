#include "../include/core.h"

int main (int argc, char const *argv[])
{
        Core* c = create_ios_client();
        inspect(c);
        Core* a = create_android_client();
        inspect(a);
}
