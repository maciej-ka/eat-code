// https://leetcode.com/problems/design-task-manager/submissions/1775275027/
import _ from 'lodash';
import {} from 'datastructures-js';

/**
 * Your TaskManager object will be instantiated and called as such:
 * var obj = new TaskManager(tasks)
 * obj.add(userId,taskId,priority)
 * obj.edit(taskId,newPriority)
 * obj.rmv(taskId)
 * var param_4 = obj.execTop()
 */
class TaskManager {
  /**
   * @param {number[][]} tasks
   */
  constructor(tasks) {
    this.heap = new TaskHeap();
    for (const task of tasks) this.heap.add(task);
  }

  /**
   * @param {number} userId
   * @param {number} taskId
   * @param {number} priority
   * @return {void}
   */
  add(userId, taskId, priority) {
    this.heap.add([userId, taskId, priority]);
  }

  /**
   * @param {number} taskId
   * @param {number} newPriority
   * @return {void}
   */
  edit(taskId, newPriority) {
    this.heap.edit(taskId, newPriority);
  }

  /**
   * @param {number} taskId
   * @return {void}
   */
  rmv(taskId) {
    this.heap.remove(taskId);
  }

  /**
   * @return {number}
   */
  execTop() {
    return this.heap.pop();
  }
}

const USER_ID = 0;
const TASK_ID = 1;
const PRIORITY = 2;

class TaskHeap {
  constructor() {
    this.array = [];
    // taskId => index
    this.lookup = {};
  }

  isGreater(i, k) {
    return this.array[i][PRIORITY] > this.array[k][PRIORITY]
      || this.array[i][PRIORITY] == this.array[k][PRIORITY] && this.array[i][TASK_ID] > this.array[k][TASK_ID];
  }

  swap(i, k) {
    this.lookup[this.array[i][TASK_ID]] = k;
    this.lookup[this.array[k][TASK_ID]] = i;

    const temp = this.array[i];
    this.array[i] = this.array[k];
    this.array[k] = temp;
  }

  bubbleUp(i) {
    if (i === 0) return;
    const p = i - 1 >> 1;
    if (this.isGreater(i, p)) {
      this.swap(i, p);
      this.bubbleUp(p);
    }
  }

  bubbleDown(i) {
    let c = (i << 1) + 1;
    if (c >= this.array.length) return;
    if (c + 1 < this.array.length && this.isGreater(c + 1, c)) c++;
    if (this.isGreater(c, i)) {
      this.swap(i, c);
      this.bubbleDown(c);
    }
  }

  add(elem) {
    this.lookup[elem[TASK_ID]] = this.array.length;
    this.array.push(elem);
    this.bubbleUp(this.array.length - 1);
  }

  remove(taskId) {
    const i = this.lookup[taskId];
    this.array[i][PRIORITY] = Number.MAX_SAFE_INTEGER;
    this.bubbleUp(i);
    this.pop();
  }

  edit(taskId, priority) {
    const i = this.lookup[taskId];
    const oldPriority = this.array[i][PRIORITY];
    this.array[i][PRIORITY] = priority;
    if (priority > oldPriority) return this.bubbleUp(i);
    if (priority < oldPriority) return this.bubbleDown(i);
  }

  pop() {
    if (this.array.length === 0) return -1;
    const result = this.array[0][USER_ID];
    this.array[0] = this.array[this.array.length - 1];
    this.lookup[this.array[0][TASK_ID]] = 0;
    this.array.pop();
    this.bubbleDown(0);
    return result;
  }

  debug() {
    console.log("--------------");
    console.log(this.array);
    console.log(this.lookup);
    console.log();
  }
}

export default TaskManager;
