/*
    Author: OrzMiku
    A unique_ptr and shared_ptr implementation in C++. Idk if it's correct. I never take a look at the cpp standard library.
*/

template <typename T>
class unique_ptr
{
private:
    T *ptr_;

public:
    explicit unique_ptr(T *ptr) : ptr_(ptr) {}

    unique_ptr(const unique_ptr &) = delete;
    unique_ptr &operator=(const unique_ptr &) = delete;

    unique_ptr(unique_ptr &&other) : ptr_(other.ptr_)
    {
        other.ptr_ = nullptr;
    }

    unique_ptr &operator=(unique_ptr &&other)
    {
        if (this != &other)
        {
            delete ptr_;
            ptr_ = other.ptr_;
            other.ptr_ = nullptr;
        }
        return *this;
    }

    ~unique_ptr() { delete ptr_; }

    T &operator*() { return *ptr_; }
    T *operator->() { return ptr_; }
    T &operator*() const { return *ptr_; }
    T *operator->() const { return ptr_; }
    T *get() const { return ptr_; }

    T *release() {
        T *ptr = ptr_;
        ptr_ = nullptr;
        return ptr;
    }

    void reset(T *ptr = nullptr)
    {
        if (ptr_ != ptr)
        {
            delete ptr_;
            ptr_ = ptr;
        }
    }
};

template <typename T>
class shared_ptr
{
private:
    T *ptr_;
    size_t *ref_count_;
    void release()
    {
        if (ptr_ && --(*ref_count_) == 0)
        {
            delete ptr_;
            delete ref_count_;
        }
    }

public:
    explicit shared_ptr(T *ptr) : ptr_(ptr), ref_count_(new size_t(1)) {}

    shared_ptr(const shared_ptr &other) : ptr_(other.ptr_), ref_count_(other.ref_count_)
    {
        ++(*ref_count_);
    }

    shared_ptr &operator=(const shared_ptr &other)
    {
        if (this != &other)
        {
            release();
            ptr_ = other.ptr_;
            ref_count_ = other.ref_count_;
            ++(*ref_count_);
        }
        return *this;
    }

    shared_ptr(shared_ptr &&other) : ptr_(other.ptr_), ref_count_(other.ref_count_)
    {
        other.ptr_ = nullptr;
        other.ref_count_ = nullptr;
    }

    shared_ptr &operator=(shared_ptr &&other)
    {
        if (this != &other)
        {
            release();
            ptr_ = other.ptr_;
            ref_count_ = other.ref_count_;
            other.ptr_ = nullptr;
            other.ref_count_ = nullptr;
        }
        return *this;
    }

    ~shared_ptr() { release(); }

    T &operator*() { return *ptr_; }
    T *operator->() { return ptr_; }
    T &operator*() const { return *ptr_; }
    T *operator->() const { return ptr_; }
    T *get() const { return ptr_; }
    size_t use_count() const { return *ref_count_; }

    void reset(T *ptr = nullptr)
    {
        if (ptr_ != ptr)
        {
            release();
            ptr_ = ptr;
            ref_count_ = new size_t(1);
        }
    }
};