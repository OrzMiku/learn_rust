/*
    Author: OrzMiku
    A Example of RAII(Resource Acquisition Is Initialization) in C++, using unique_ptr and shared_ptr to manage resources.
    The implementation of unique_ptr and shared_ptr is in pointer.cpp. Idk if it's correct. I never take a look at the cpp standard library.
*/

#include <iostream>
#include "pointer.cpp"

// Example class
class MyClass {
public:
    MyClass(int value) : value_(value) {
        std::cout << "MyClass constructed with value: " << value_ << "\n";
    }
    ~MyClass() {
        std::cout << "MyClass destroyed with value: " << value_ << "\n";
    }
    void print() const {
        std::cout << "MyClass value: " << value_ << "\n";
    }
    void setValue(int value) {
        value_ = value;
    }
private:
    int value_;
};

// unique_ptr Demo
void unique_ptr_demo() {
    std::cout << "=== unique_ptr demo ===\n";

    // Create a unique_ptr
    unique_ptr<MyClass> ptr1(new MyClass(10));
    ptr1->print();

    // Move unique_ptr
    unique_ptr<MyClass> ptr2 = std::move(ptr1);
    if (!ptr1.get()) {
        std::cout << "ptr1 is now nullptr\n";
    }
    ptr2->print();

    // Reset unique_ptr
    ptr2.reset(new MyClass(20));
    ptr2->print();

    // Release unique_ptr ownership
    MyClass *raw_ptr = ptr2.release();
    if (!ptr2.get()) {
        std::cout << "ptr2 is now nullptr\n";
    }
    // Because we released the ownership, we need to delete the raw pointer
    delete raw_ptr;

    std::cout << "=== unique_ptr demo end ===\n\n";
}

// shared_ptr Demo
void shared_ptr_demo() {
    std::cout << "=== shared_ptr demo ===\n";

    // Create a shared_ptr
    shared_ptr<MyClass> ptr1(new MyClass(30));
    ptr1->print();
    std::cout << "ptr1 use_count: " << ptr1.use_count() << "\n"; // 输出 1

    // Copy shared_ptr
    shared_ptr<MyClass> ptr2 = ptr1;
    ptr2->print();
    std::cout << "ptr1 use_count: " << ptr1.use_count() << "\n"; // 输出 2
    std::cout << "ptr2 use_count: " << ptr2.use_count() << "\n"; // 输出 2

    // Value change
    ptr2->setValue(40);
    ptr1->print();
    ptr2->print();

    // Move shared_ptr
    shared_ptr<MyClass> ptr3 = std::move(ptr2);
    if (!ptr2.get()) {
        std::cout << "ptr2 is now nullptr\n";
    }
    ptr3->print();
    std::cout << "ptr1 use_count: " << ptr1.use_count() << "\n"; // 输出 2
    std::cout << "ptr3 use_count: " << ptr3.use_count() << "\n"; // 输出 2

    // Reset shared_ptr
    ptr3.reset(new MyClass(50));
    ptr3->print();
    std::cout << "ptr1 use_count: " << ptr1.use_count() << "\n"; // 输出 1

    std::cout << "=== shared_ptr demo end ===\n\n";
}

int main() {
    unique_ptr_demo();
    shared_ptr_demo();
    return 0;
}