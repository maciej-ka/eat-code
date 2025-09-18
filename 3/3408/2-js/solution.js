// https://leetcode.com/problems/design-task-manager/submissions/1775476768/?envType=daily-question&envId=2025-09-18
import _ from 'lodash';
import { PriorityQueue } from 'datastructures-js';

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
    // taskId => priority
    this.priorities = new Map();
    // taskId => userId
    this.userIds = new Map();

    this.heap = new PriorityQueue(
      (a, b) => a[1] === b[1]
        ? b[0] - a[0]
        : b[1] - a[1]
    )
    for (const task of tasks) {
      this.add(task[0], task[1], task[2]);
    }
  }

  /**
   * @param {number} userId
   * @param {number} taskId
   * @param {number} priority
   * @return {void}
   */
  add(userId, taskId, priority) {
    this.userIds.set(taskId, userId);
    this.heap.push([taskId, priority]);
    this.priorities.set(taskId, priority);
  }

  /**
   * @param {number} taskId
   * @param {number} newPriority
   * @return {void}
   */
  edit(taskId, newPriority) {
    this.heap.push([taskId, newPriority]);
    this.priorities.set(taskId, newPriority);
  }

  /**
   * @param {number} taskId
   * @return {void}
   */
  rmv(taskId) {
    this.priorities.set(taskId, -1);
  }

  /**
   * @return {number}
   */
  execTop() {
    while(true) {
      const e = this.heap.pop();
      if (e === null) return -1;
      if (e[1] === this.priorities.get(e[0])) {
        this.priorities.delete(e[0]);
        return this.userIds.get(e[0]);
      }
    }
  }
}

export default TaskManager;
