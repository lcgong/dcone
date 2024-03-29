
### 改进计划
* NodeValue 直接存在log里， Map和List的值是Log的索引位置
* 索引位置采用递增整数，与log的vec索引差别在偏移量
* List语义上，减弱vec化，插入，前后追加对List的影响不大，数组下标应该看成额外的访问索引
* focus的列表整数下标应该是相对下标，起到方便访问列表元素的临时“名字”的作用，下标是有可能在下一时刻就会发生偏移的
* 列表的下标的是相对位置，而不是永久的绝对位置，一般相对的是第一个元素
* 在List下标里引入两个特殊下标，头部下标Head和尾部下标Tail
* List可以看做是Map的扩展，除了父子方向外（被标记），还有线性横向的兄弟关系（有序），相对尾倒序，相对头正向
* 不在使用Focus树的方式，每一个Node，有一个AccessKey
* 不再支持绝对路径的思路，路径(access_path)是相对Node的，相对“根root”就所谓的传统意义的绝对路径。
* 列表#1表示相对头后的第一个列表项目，#0列表头自己，#-1列表尾项目之前第一个项目，
* 不支持path方法，但有方法偏移+1相对访问列表元素的之后一个，-1相对访问元素之前一个