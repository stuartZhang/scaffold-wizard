/**
 * 和原版 inquirer 的微小差别，这里的问题清单不是 Array<inquirer.DistinctQuestion<T>>，
 * 而是一个 Object<inquirer.KeyUnion<T>, inquirer.DistinctQuestion<T>>。其中，键是一个问题
 * 的唯一标识符 identifier（等同于【问题配置对象】里的 name 属性）；值就是该问题的配置对象。
 *
 * 另一方面，问题的提问次序与 Questions 配置对象内【键-值】对的词法次序一致。
 * @export
 * @interface Questions
 * @template T
 */
export interface Questions<T> {
    [question_name: inquirer.KeyUnion<T>]: inquirer.DistinctQuestion<T>
}
/**
 * 在经由图形界面收集问卷答案时，阻塞 libuv 的事件循环。
 * @export
 * @param {Questions} questions
 * @returns {Promise<inquirer.Answers>}
 */
export function inquire(questions: Questions): Promise<inquirer.Answers>;
/**
 * 在经由图形界面收集问卷答案时，不阻塞 libuv 的事件循环。
 * @export
 * @param {Questions} questions
 * @returns {Promise<inquirer.Answers>}
 */
export function inquireAsync(questions: Questions): Promise<inquirer.Answers>;

